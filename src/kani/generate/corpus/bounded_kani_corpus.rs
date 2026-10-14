//! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
//!
//! CG owns the semantic-family lowerers. This module owns the codegen-side
//! vertical slice: once one lowering is admitted, it renders the corpus roles (oracle, strategy,
//! Kani harness, QSL source and dependency, and proof-dependency graph) from the same profile
//! selection, finite input, and declared proof-dependency census. A non-success outcome returns before any role is
//! emitted.

use std::{collections::BTreeSet, fmt::Write as _};

use quire_canonical::FixedShape;
use quire_contract_ir::kani::{
    DispatchIndex, FiniteInput, FiniteObject, FiniteReference, KaniOutcome, KaniOutcomeError,
    KaniOutcomeKind, KaniProfile, ProfileSelection, ValidatedFiniteInput,
};
use quire_contract_model::{std001_code, Std001Code};
use serde::{Deserialize, Serialize};

use crate::{
    core::identity::HarnessSymbol,
    core::{
        artifact::Artifact,
        canonical::{
            artifact_json, content_digest, owner_input_value, preimage_shape_changed,
            preimage_value, DigestError,
        },
    },
    kani::census::{
        dependency_readiness, normalize_dependencies, ProofDependencyEdge, ProofDependencyKind,
        ProofDependencyRequest, ProofReadiness,
    },
    kani::generate::census_validation::validate_dependencies,
    kani::generate::lower::{
        bounded_collections::{prepare_bounded_collection_query, CollectionQuery, QueryKind},
        definedness_arithmetic::{
            prepare_checked_arithmetic, ArithmeticLowering, CheckedArithmeticRequest,
        },
        finite_reference_graphs::{prepare_finite_graph_reaches, GraphLowering, GraphRequest},
        FamilyLoweringError,
    },
};

/// Stable schema identity for [`CorpusProofDependencyGraph`].
///
/// Deliberately distinct from the V1 bundle's `quire.kani-proof-graph/v2`
/// ([`crate::kani::generate::v1_bundle::ProofDependencyGraph`], validated against
/// `schemas/kani-proof-graph-v2.schema.json`): that schema requires a Contract-IR `requirementId`/
/// `requirementRevision` this corpus's finite-ABI input has no analogue for, and requires a
/// cargo-kani CLI `options` array of at least fifteen entries that this generator never builds
/// (`corpus`'s harnesses are plain `#[kani::proof]`, not `#[kani::proof_for_contract]`, and no
/// unwind/solver/harness-filter option vector is ever assembled for them). Reusing that exact
/// envelope here would mean fabricating those fields; this schema instead carries only what
/// [`generate_bounded_kani_corpus_case`] actually derives (ir#80).
pub const CORPUS_PROOF_GRAPH_SCHEMA: &str = "quire.kani-corpus-proof-graph/v1";

/// One semantic family represented in the bounded corpus.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedCorpusFamily {
    /// Checked arithmetic and its definedness boundary.
    DefinednessArithmetic,
    /// Identity-preserving positive-length reachability.
    FiniteReferenceGraph,
    /// Ordered, duplicate-preserving finite collection queries.
    BoundedCollection,
}

impl BoundedCorpusFamily {
    const fn construct(self) -> &'static str {
        match self {
            Self::DefinednessArithmetic => "checked-arithmetic",
            Self::FiniteReferenceGraph => "finite-reference-graph",
            Self::BoundedCollection => "bounded-collection-query",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::DefinednessArithmetic => "arithmetic",
            Self::FiniteReferenceGraph => "graph",
            Self::BoundedCollection => "collection",
        }
    }
}

/// One exact semantic request selected for corpus generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedCorpusRequest {
    /// A checked arithmetic operation.
    Arithmetic(CheckedArithmeticRequest),
    /// A finite positive-length reachability query.
    Graph(GraphRequest),
    /// An ordered finite collection query.
    Collection(CollectionQuery),
}

impl BoundedCorpusRequest {
    const fn family(&self) -> BoundedCorpusFamily {
        match self {
            Self::Arithmetic(_) => BoundedCorpusFamily::DefinednessArithmetic,
            Self::Graph(_) => BoundedCorpusFamily::FiniteReferenceGraph,
            Self::Collection(_) => BoundedCorpusFamily::BoundedCollection,
        }
    }

    fn source_id(&self) -> &str {
        match self {
            Self::Arithmetic(request) => request.source_id,
            Self::Graph(request) => &request.source_id,
            Self::Collection(request) => &request.source_id,
        }
    }
}

/// Deterministic generated artifacts for one admitted finite corpus case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedCorpusArtifacts {
    /// Boolean executable oracle artifact.
    pub oracle: Artifact,
    /// Finite, directly shaped strategy artifact.
    pub strategy: Artifact,
    /// Kani harness artifact with no input-erasing assumptions.
    pub kani_harness: Artifact,
    /// Real QSL source for this exact finite case; replay compiles these bytes through
    /// `qsl_replay::call_site` rather than supplying a copied package.
    pub qsl_source: Artifact,
    /// The locked QSL library source imported by `qsl_source` for this case.
    pub qsl_dependency: Artifact,
    /// Deterministic corpus-scoped proof-dependency graph for this case
    /// ([`CORPUS_PROOF_GRAPH_SCHEMA`]).
    pub proof_graph: Artifact,
}

/// Deterministic, corpus-scoped proof-dependency graph for one bounded-Kani corpus case.
///
/// Carries only what [`generate_bounded_kani_corpus_case`] actually derives: the case's own
/// `#[kani::proof]` symbol, its semantic family/construct, its case name (the same name its
/// sibling artifact paths carry), the derived readiness, and the sorted declared
/// dependency census. See [`CORPUS_PROOF_GRAPH_SCHEMA`] for why this is a distinct envelope from
/// `src/kani/generate/v1_bundle.rs`'s `quire.kani-proof-graph/v2`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CorpusProofDependencyGraph {
    /// Required resource ceilings of this concrete-case proof.
    pub ceilings: crate::kani::identity::ProofCeilings,
    /// Stable graph schema identity ([`CORPUS_PROOF_GRAPH_SCHEMA`]).
    pub schema_version: String,
    /// The corpus case's own `#[kani::proof]` symbol; identical to the suffix-bearing function
    /// name in [`BoundedCorpusArtifacts::kani_harness`].
    pub proof_id: String,
    /// Semantic family label (e.g. `arithmetic`).
    pub family: String,
    /// Contract IR construct name this case lowers (e.g. `checked-arithmetic`).
    pub construct: String,
    /// The corpus case's name, `{family}_{digest}`; the same name its artifact paths carry.
    pub identity: String,
    /// Derived dependency readiness; this generator never executes or classifies a proof.
    pub readiness: ProofReadiness,
    /// Sorted complete declared dependency census.
    pub dependencies: Vec<ProofDependencyEdge>,
}

impl crate::core::canonical::ArtifactJsonRecord for CorpusProofDependencyGraph {}

/// Registry of the corpus case identities emitted into one output tree.
///
/// A case's identity is the SHA-256 of its canonical request content: the
/// request, the finite input it is evaluated over, the profile selection, and the normalized
/// proof-dependency census. Every emitted artifact path is `corpus/{family}_{identity}.*`, so the
/// same content always lands on the same files and two distinct cases never share one, whatever
/// the emission order. Callers generating more than one case into the same output tree share one
/// registry; a case whose identity it already holds is refused as
/// `kani_corpus_identity_collision` rather than emitted again. The identity is recorded only once
/// every fallible step of [`generate_bounded_kani_corpus_case`] has succeeded, so a refused case
/// never claims one.
#[derive(Debug, Default)]
pub struct EmittedCorpusIdentities(BTreeSet<String>);

impl EmittedCorpusIdentities {
    /// Starts an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records `identity`; `false` when it was already held.
    fn claim(&mut self, identity: &str) -> bool {
        self.0.insert(identity.to_owned())
    }
}

/// The canonical content one corpus case is identified by.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CaseIdentity<'a> {
    ceilings: crate::kani::identity::ProofCeilings,
    construct: &'static str,
    profile: &'a ProfileSelection,
    input: serde_json::Value,
    request: RequestIdentity<'a>,
    dependencies: &'a [ProofDependencyEdge],
}

/// Every field of a [`BoundedCorpusRequest`], including those the rendered oracle does not read.
#[derive(Serialize, FixedShape)]
#[serde(rename_all = "camelCase")]
enum RequestIdentity<'a> {
    Arithmetic {
        source_id: &'a str,
        operator: &'static str,
        left: String,
        right: String,
        minimum: String,
        maximum: String,
    },
    Graph {
        source_id: &'a str,
        start_id: &'a str,
        target_id: &'a str,
        field_id: &'a str,
        max_expansions: usize,
    },
    Collection {
        source_id: &'a str,
        values: Vec<String>,
        max_items: usize,
        kind: QueryKindIdentity,
    },
}

#[derive(Serialize, FixedShape)]
#[serde(rename_all = "camelCase")]
enum QueryKindIdentity {
    ForAllNonNegative,
    ExistsEqual(String),
}

impl<'a> From<&'a BoundedCorpusRequest> for RequestIdentity<'a> {
    // Each request struct is destructured without `..`, so a field added to one is a compile
    // error here rather than a field silently left out of the identity.
    fn from(request: &'a BoundedCorpusRequest) -> Self {
        match request {
            BoundedCorpusRequest::Arithmetic(request) => {
                let CheckedArithmeticRequest {
                    source_id,
                    operator,
                    left,
                    right,
                    minimum,
                    maximum,
                } = request;
                Self::Arithmetic {
                    source_id,
                    operator: checked_method(*operator),
                    left: left.to_string(),
                    right: right.to_string(),
                    minimum: minimum.to_string(),
                    maximum: maximum.to_string(),
                }
            }
            BoundedCorpusRequest::Graph(request) => {
                let GraphRequest {
                    source_id,
                    start_id,
                    target_id,
                    field_id,
                    max_expansions,
                } = request;
                Self::Graph {
                    source_id,
                    start_id,
                    target_id,
                    field_id,
                    max_expansions: *max_expansions,
                }
            }
            BoundedCorpusRequest::Collection(request) => {
                let CollectionQuery {
                    source_id,
                    values,
                    max_items,
                    kind,
                } = request;
                Self::Collection {
                    source_id,
                    values: values.iter().map(ToString::to_string).collect(),
                    max_items: *max_items,
                    kind: match kind {
                        QueryKind::ForAllNonNegative => QueryKindIdentity::ForAllNonNegative,
                        QueryKind::ExistsEqual(expected) => {
                            QueryKindIdentity::ExistsEqual(expected.to_string())
                        }
                    },
                }
            }
        }
    }
}

/// Starts from the owning finite-input serializer. The case-name contract retains three
/// camelCase key names and normalizes the two collections whose offered order does not affect
/// the rendered graph. All values come from the original input serializer or its typed members.
fn normalized_input_value(input: &FiniteInput) -> Result<serde_json::Value, DigestError> {
    let mut value = owner_input_value(input)?;
    let root = value
        .as_object_mut()
        .ok_or_else(|| preimage_shape_changed("finite input is not an object"))?;

    // The case-name preimage predates the owning input wire names for these three fields.
    // Retain its committed camelCase keys while taking each value from the owner serializer.
    for (source, identity) in [
        ("input_bytes", "inputBytes"),
        ("model_id", "modelId"),
        ("source_id", "sourceId"),
    ] {
        if root.contains_key(identity) {
            return Err(preimage_shape_changed(
                "finite input has a conflicting case-identity key",
            ));
        }
        let field = root
            .remove(source)
            .ok_or_else(|| preimage_shape_changed("finite input identity field is absent"))?;
        root.insert(identity.to_owned(), field);
    }

    let objects_slot = root
        .get_mut("objects")
        .filter(|slot| slot.is_array())
        .ok_or_else(|| {
            preimage_shape_changed("finite input objects field is absent or not an array")
        })?;
    let mut objects: Vec<_> = input
        .objects
        .iter()
        .map(|object| {
            let FiniteObject {
                identity,
                type_id,
                snapshot_id,
            } = object;
            (identity.as_str(), type_id.as_str(), snapshot_id.as_str())
        })
        .collect();
    objects.sort_unstable();
    *objects_slot = preimage_value(&objects)?;

    let references_slot = root
        .get_mut("references")
        .filter(|slot| slot.is_array())
        .ok_or_else(|| {
            preimage_shape_changed("finite input references field is absent or not an array")
        })?;
    let mut references: Vec<_> = input
        .references
        .iter()
        .map(|reference| {
            let FiniteReference {
                source_id,
                field_id,
                target_id,
            } = reference;
            (source_id.as_str(), field_id.as_str(), target_id.as_str())
        })
        .collect();
    references.sort_unstable();
    *references_slot = preimage_value(&references)?;
    Ok(value)
}

impl CaseIdentity<'_> {
    /// Lowercase hex SHA-256 over this content's RFC 8785 bytes, without a newline.
    fn digest(&self) -> Result<String, DigestError> {
        let digest = content_digest(&preimage_value(self)?)?;
        Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
    }
}

/// Complete codegen result for one supported corpus case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedCorpusCase {
    /// Semantic family selected before lowering.
    pub family: BoundedCorpusFamily,
    /// Exact Contract IR outcome shared by native/oracle/strategy/Kani case execution.
    pub outcome: KaniOutcome,
    /// The exact Kani symbol whose playback is joined to this case's QSL source.
    pub(crate) harness_symbol: HarnessSymbol,
    /// Generated roles derived from the same profile and finite input selection.
    pub artifacts: BoundedCorpusArtifacts,
}

impl BoundedCorpusCase {
    /// Replays actual Kani counterexample playback against this case's retained QSL package.
    ///
    /// The package is compiled from the generated unit and imported dependency source each time;
    /// the harness identity is then tied to QSL's selected function site.
    pub fn replay_counterexample(
        &self,
        transcript: &str,
    ) -> Result<crate::ReplayVerdict, BoundedCorpusReplayError> {
        let package = compile_qsl_package(self).map_err(BoundedCorpusReplayError::Package)?;
        if !package.source_matches(self.artifacts.qsl_source.contents.as_bytes()) {
            return Err(BoundedCorpusReplayError::SourceMismatch);
        }
        let identity = package
            .harness_identity(None, self.harness_symbol.clone(), Vec::new())
            .map_err(|error| {
                BoundedCorpusReplayError::Replay(
                    crate::replay::function::SpineReplayError::Identity(error),
                )
            })?;
        crate::replay::function::replay_harness_counterexample(&identity, transcript, &package)
            .map_err(BoundedCorpusReplayError::Replay)
    }
}

/// Why a retained corpus case could not be replayed against its QSL source.
#[derive(Debug)]
pub enum BoundedCorpusReplayError {
    /// The retained unit/dependency package did not compile or select the corpus function.
    Package(crate::replay::function::ReplayPackageError),
    /// The package source did not match the case's retained source bytes.
    SourceMismatch,
    /// Kani playback could not be decoded or QSL refused the witness request.
    Replay(crate::replay::function::SpineReplayError),
}

/// Why [`generate_bounded_kani_corpus_case`] emitted no case.
///
/// A refusal is almost always a typed Contract IR [`KaniOutcome`] ([`BoundedCorpusError::Outcome`]),
/// whether Contract IR's own lowering raised it or this module did. The other variant exists
/// because `KaniOutcome::non_success` is fallible for the two kinds that carry a Boolean claim:
/// this module only ever asks for `Refused` and `InvalidInput`, so
/// [`BoundedCorpusError::OutcomeConstruction`] is not reachable from the code below, but it is a
/// typed value and not a panic or a silently substituted outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedCorpusError {
    /// A typed non-success outcome; no artifact was emitted.
    Outcome(KaniOutcome),
    /// Contract IR refused to build a non-success outcome (`kani_outcome_invalid`); no artifact
    /// was emitted.
    ///
    /// CG cannot produce this today: it asks `KaniOutcome::non_success` only for `Refused` and
    /// `InvalidInput`, and Contract IR exposes no infallible per-kind constructor to a consumer.
    /// A caller that receives it should treat it as a defect in this crate.
    OutcomeConstruction(KaniOutcomeError),
}

impl BoundedCorpusError {
    /// The non-success outcome, or `None` when Contract IR refused to build one.
    #[must_use]
    pub const fn outcome(&self) -> Option<&KaniOutcome> {
        match self {
            Self::Outcome(outcome) => Some(outcome),
            Self::OutcomeConstruction(_) => None,
        }
    }

    /// The stable STD-001 code of this refusal.
    #[must_use]
    pub const fn code(&self) -> Std001Code {
        match self {
            Self::Outcome(outcome) => outcome.code,
            Self::OutcomeConstruction(error) => error.code(),
        }
    }
}

impl From<KaniOutcome> for BoundedCorpusError {
    fn from(outcome: KaniOutcome) -> Self {
        Self::Outcome(outcome)
    }
}

impl From<KaniOutcomeError> for BoundedCorpusError {
    fn from(error: KaniOutcomeError) -> Self {
        Self::OutcomeConstruction(error)
    }
}

impl From<FamilyLoweringError> for BoundedCorpusError {
    fn from(error: FamilyLoweringError) -> Self {
        match error {
            FamilyLoweringError::Outcome(outcome) => Self::Outcome(outcome),
            FamilyLoweringError::OutcomeConstruction(error) => Self::OutcomeConstruction(error),
        }
    }
}

impl std::fmt::Display for BoundedCorpusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Outcome(outcome) => write!(
                f,
                "{}: {:?} for source {:?} in context {:?}",
                outcome.code, outcome.kind, outcome.source_id, outcome.context
            ),
            Self::OutcomeConstruction(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for BoundedCorpusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Outcome(_) => None,
            Self::OutcomeConstruction(error) => Some(error),
        }
    }
}

/// The two non-success kinds this module raises. Naming only these keeps a `proved` or
/// `counterexample` request, the one input `KaniOutcome::non_success` refuses, unwritable here.
#[derive(Clone, Copy)]
enum CorpusRefusal {
    /// The caller's input is not acceptable.
    InvalidInput,
    /// The input is acceptable and generation could not complete.
    Refused,
}

impl CorpusRefusal {
    /// Builds this module's refusal of `code` for one request.
    fn raise(
        self,
        code: Std001Code,
        source_id: impl Into<String>,
        context: impl Into<String>,
    ) -> BoundedCorpusError {
        let kind = match self {
            Self::InvalidInput => KaniOutcomeKind::InvalidInput,
            Self::Refused => KaniOutcomeKind::Refused,
        };
        match KaniOutcome::non_success(kind, code, source_id, context) {
            Ok(outcome) => BoundedCorpusError::Outcome(outcome),
            Err(error) => BoundedCorpusError::OutcomeConstruction(error),
        }
    }
}

/// Generates the complete codegen corpus case from one already validated Contract IR input.
///
/// A refusal is a [`BoundedCorpusError`]: [`BoundedCorpusError::Outcome`] wraps the typed
/// [`KaniOutcome`] described below, and [`BoundedCorpusError::OutcomeConstruction`] is Contract
/// IR's refusal to build a non-success outcome, which this function cannot produce today.
///
/// Contract IR validates the finite population; CG checks profile/dispatch and lowers semantics first.
/// Consequently a refused, invalid, incomplete, or exhausted case returns its original typed
/// outcome and this function emits no artifact.  Generated Kani source uses a concrete case and
/// intentionally contains no `kani::assume` call.
///
/// `dependencies` is the caller-declared proof-dependency census for this exact case. It is
/// validated with the same rules `generate_kani_bundle` applies to its own census
/// (`crate::kani::generate::census_validation::validate_dependencies`),
/// plus a Required-only rule this corpus adds on top: the corpus's generated harnesses are
/// self-contained by construction (literal operands/edges baked in at generation time, no external
/// call, no `// proof-dependency-site:` marker, no `kani::assume`, no `#[kani::stub]`), so any
/// declared `Assumed` or `Stubbed` edge is refused rather than accepted and fabricated a
/// `sourceSite` pointing at a marker/assume/stub this generator never renders (ir#80 review finding
/// F1). A valid census is retained in full as the case's own
/// [`BoundedCorpusArtifacts::proof_graph`] artifact. An empty census is the ordinary case and
/// yields a `Ready` [`ProofReadiness`]; an invalid census -- an empty or duplicate declared
/// identity, an inconsistent kind/state/path combination, or any non-`Required` kind -- returns a
/// typed `InvalidInput` `kani_corpus_dependency_invalid` result before any artifact is emitted.
///
/// The case is named from its canonical content digest ([`EmittedCorpusIdentities`]), so identical
/// requests name identical artifacts and distinct requests never share a path. `emitted` records
/// that identity only after every other fallible step has succeeded; a case whose identity it
/// already holds returns `InvalidInput` `kani_corpus_identity_collision` and emits nothing. A failure
/// to serialize the case's proof-dependency graph returns `Refused`
/// `kani_corpus_serialization_failed`, emits nothing and records no identity.
pub fn generate_bounded_kani_corpus_case(
    ceilings: crate::kani::identity::ProofCeilings,
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    request: BoundedCorpusRequest,
    dependencies: &[ProofDependencyRequest<'_>],
    emitted: &mut EmittedCorpusIdentities,
) -> Result<BoundedCorpusCase, BoundedCorpusError> {
    if profile.selection() != &input.input().profile {
        return Err(CorpusRefusal::InvalidInput.raise(
            std001_code!("kani_profile_input_mismatch"),
            request.source_id(),
            profile.selection().revision.clone(),
        ));
    }
    let family = request.family();
    let request_source_id = request.source_id().to_owned();
    let revision = profile.selection().revision.clone();
    // The declared census is validated, and its normalized/sorted form is fixed, before any
    // lowering begins: an invalid census is refused before the generator does any semantic work,
    // and the single normalized form computed here is the one the emitted proof-dependency-graph
    // artifact (`render_artifacts`) carries.
    //
    // Validation is two layers. First, the shared rules (`validate_dependencies`)
    // -- non-empty and unique declared identities, and a closed kind/state/path shape per entry.
    // Second, this corpus's own Required-only rule (ir#80 review finding F1): the corpus's
    // generated harnesses are self-contained by construction -- rendering no
    // `// proof-dependency-site:` marker, no `kani::assume`, and no `#[kani::stub]` -- so an
    // `Assumed` or `Stubbed` declared edge would fabricate an `assumption:<id>`/`stub:<id>`
    // `sourceSite` pointing at a marker this generator never emits. Both layers report the
    // identical typed refusal below, since a caller cannot act differently on either failure.
    //
    // `validate_dependencies`'s second argument is the "root proof id" a dependency must not name
    // itself; passing `request_source_id` here would spuriously refuse a legitimate dependency
    // whose declared proof id happens to equal the request's own source id, even though a real
    // self-dependency cannot occur -- this corpus case's own harness symbol is named from its
    // content digest, which is claimed only after this census is accepted. The empty string is passed
    // instead: `validate_dependencies` requires every declared proof id to be non-empty, so `""`
    // can never equal a legitimate one and the self-dependency check can never spuriously fire
    // (ir#80 review finding F5).
    let declared_dependencies_invalid = dependencies
        .iter()
        .any(|dependency| dependency.kind != ProofDependencyKind::Required)
        || validate_dependencies(dependencies, "").is_err();
    if declared_dependencies_invalid {
        // The failing diagnostic's own `path` (which census index) and `message` (which rule) are
        // not carried into this refusal: `KaniOutcome::non_success`'s `source_id`/`context` fields
        // already carry `request_source_id`/`revision` -- the request's own identity, not the
        // census's -- and folding diagnostic detail into either would conflate two different
        // things this outcome identifies. A caller that needs the failing census index re-runs
        // `validate_dependencies` directly over the same census for the full
        // diagnostic list (ir#80 review finding F7).
        return Err(CorpusRefusal::InvalidInput.raise(
            std001_code!("kani_corpus_dependency_invalid"),
            request_source_id,
            revision,
        ));
    }
    let normalized_dependencies = normalize_dependencies(dependencies);
    // Computed before lowering consumes the request; recorded only after lowering succeeds.
    let input_identity = normalized_input_value(input.input()).map_err(|_| {
        CorpusRefusal::InvalidInput.raise(
            std001_code!("kani_corpus_identity_unencodable"),
            request_source_id.clone(),
            revision.clone(),
        )
    })?;
    let identity = CaseIdentity {
        ceilings,
        construct: family.construct(),
        profile: profile.selection(),
        input: input_identity,
        request: RequestIdentity::from(&request),
        dependencies: &normalized_dependencies,
    }
    .digest()
    .map_err(|_| {
        CorpusRefusal::InvalidInput.raise(
            std001_code!("kani_corpus_identity_unencodable"),
            request_source_id.clone(),
            revision.clone(),
        )
    })?;
    let (value, oracle_body, qsl_body) = match request {
        BoundedCorpusRequest::Arithmetic(request) => {
            let lowered = prepare_checked_arithmetic(profile, dispatch, input, request)?;
            // Admission establishes the checked arithmetic/definedness property; the numeric
            // result itself is not a Boolean verdict (zero is as valid as any other in-range
            // result).
            (
                true,
                render_arithmetic_oracle(&lowered),
                render_arithmetic_qsl(&lowered),
            )
        }
        BoundedCorpusRequest::Graph(request) => {
            let lowered = prepare_finite_graph_reaches(profile, dispatch, input, request)?;
            let qsl_body = render_graph_qsl(&lowered, input).ok_or_else(|| {
                CorpusRefusal::Refused.raise(
                    std001_code!("kani_corpus_serialization_failed"),
                    request_source_id.clone(),
                    revision.clone(),
                )
            })?;
            (
                lowered.reachable,
                render_graph_oracle(&lowered, input),
                qsl_body,
            )
        }
        BoundedCorpusRequest::Collection(request) => {
            let lowered = prepare_bounded_collection_query(profile, dispatch, input, request)?;
            let values = lowered
                .query
                .values
                .iter()
                .map(|value| format!("{value}i128"))
                .collect::<Vec<_>>()
                .join(", ");
            let predicate = match lowered.query.kind {
                QueryKind::ForAllNonNegative => {
                    "values.iter().all(|value| *value >= 0i128)".to_owned()
                }
                QueryKind::ExistsEqual(expected) => {
                    format!("values.iter().any(|value| *value == {expected}i128)")
                }
            };
            (
                lowered.value,
                format!("{{ let values = [{values}]; {predicate} }}"),
                render_collection_qsl(&lowered.query.values, lowered.query.kind),
            )
        }
    };
    let outcome = if value {
        KaniOutcome::proved(
            request_source_id.clone(),
            profile.selection().revision.clone(),
        )
    } else {
        KaniOutcome::counterexample(
            request_source_id.clone(),
            profile.selection().revision.clone(),
        )
    };
    // Rendered before the identity is claimed, so a refused rendering records nothing.
    let name = format!("{}_{identity}", family.label());
    let harness_symbol = HarnessSymbol::try_from(format!("corpus_case_{name}")).map_err(|_| {
        CorpusRefusal::Refused.raise(
            std001_code!("kani_corpus_serialization_failed"),
            request_source_id.clone(),
            revision.clone(),
        )
    })?;
    let Ok(artifacts) = render_artifacts(
        ceilings,
        family,
        &name,
        value,
        &oracle_body,
        &qsl_body,
        &normalized_dependencies,
    ) else {
        // The caller's input is not at fault, so this is `Refused` and not `InvalidInput`.
        return Err(CorpusRefusal::Refused.raise(
            std001_code!("kani_corpus_serialization_failed"),
            request_source_id,
            revision,
        ));
    };
    if !emitted.claim(&identity) {
        return Err(CorpusRefusal::InvalidInput.raise(
            std001_code!("kani_corpus_identity_collision"),
            request_source_id,
            revision,
        ));
    }
    Ok(BoundedCorpusCase {
        family,
        outcome,
        harness_symbol,
        artifacts,
    })
}

const fn checked_method(operator: quire_contract_model::NumericOperator) -> &'static str {
    match operator {
        quire_contract_model::NumericOperator::Add => "checked_add",
        quire_contract_model::NumericOperator::Subtract => "checked_sub",
        quire_contract_model::NumericOperator::Multiply => "checked_mul",
        quire_contract_model::NumericOperator::Divide => "checked_div",
        quire_contract_model::NumericOperator::Remainder => "checked_rem",
    }
}

fn render_arithmetic_oracle(lowered: &ArithmeticLowering) -> String {
    let operator = checked_method(lowered.request.operator);
    format!(
        "{}i128.{operator}({}i128).is_some_and(|value| value >= {}i128 && value <= {}i128)",
        lowered.request.left,
        lowered.request.right,
        lowered.request.minimum,
        lowered.request.maximum,
    )
}

fn render_arithmetic_qsl(lowered: &ArithmeticLowering) -> String {
    let operator = match lowered.request.operator {
        quire_contract_model::NumericOperator::Add => "+",
        quire_contract_model::NumericOperator::Subtract => "-",
        quire_contract_model::NumericOperator::Multiply => "*",
        quire_contract_model::NumericOperator::Divide => "/",
        quire_contract_model::NumericOperator::Remainder => "%",
    };
    let calculation = format!(
        "({} {operator} {})",
        lowered.request.left, lowered.request.right
    );
    format!(
        "({calculation} >= {}) and ({calculation} <= {})",
        lowered.request.minimum, lowered.request.maximum
    )
}

fn render_collection_qsl(values: &[i128], kind: QueryKind) -> String {
    match kind {
        QueryKind::ForAllNonNegative => default_if_empty(
            values
                .iter()
                .map(|value| format!("({value} >= 0)"))
                .collect::<Vec<_>>()
                .join(" and "),
            "true",
        ),
        QueryKind::ExistsEqual(expected) => default_if_empty(
            values
                .iter()
                .map(|value| format!("({value} = {expected})"))
                .collect::<Vec<_>>()
                .join(" or "),
            "false",
        ),
    }
}

fn render_graph_qsl(lowered: &GraphLowering, input: &ValidatedFiniteInput) -> Option<String> {
    let mut objects = input
        .input()
        .objects
        .iter()
        .map(|object| object.identity.as_str())
        .collect::<Vec<_>>();
    objects.sort_unstable();
    let indices = objects
        .iter()
        .enumerate()
        .map(|(index, identity)| (*identity, index))
        .collect::<std::collections::BTreeMap<_, _>>();
    let edges = input
        .input()
        .references
        .iter()
        .filter(|edge| edge.field_id == lowered.request.field_id)
        .map(|edge| {
            Some((
                *indices.get(edge.source_id.as_str())?,
                *indices.get(edge.target_id.as_str())?,
            ))
        })
        .collect::<Option<BTreeSet<_>>>()?;
    let vertices = objects.len();
    let adjacency = default_if_empty(
        edges
            .iter()
            .map(|(source, target)| format!("(source = {source} and target = {target})"))
            .collect::<Vec<_>>()
            .join(" or "),
        "false",
    );
    let target = *indices.get(lowered.request.target_id.as_str())?;
    let reachability = default_if_empty(
        lowered
            .expanded
            .iter()
            .map(|identity| {
                Some(format!(
                    "edge({}, {target})",
                    indices.get(identity.as_str())?
                ))
            })
            .collect::<Option<Vec<_>>>()?
            .join(" or "),
        "false",
    );
    Some(format!(
        "function edge using v(source: Int[0, {last}], target: Int[0, {last}]): Boolean pure {{ {adjacency} }}\n\
         function corpus_case using v(enabled: Boolean): Boolean pure {{ enabled and ({reachability}) }}",
        last = vertices.saturating_sub(1),
    ))
}

fn default_if_empty(expression: String, fallback: &str) -> String {
    if expression.is_empty() {
        fallback.to_owned()
    } else {
        expression
    }
}

fn render_graph_oracle(lowered: &GraphLowering, input: &ValidatedFiniteInput) -> String {
    let mut edges = input
        .input()
        .references
        .iter()
        .filter(|edge| edge.field_id == lowered.request.field_id)
        .collect::<Vec<_>>();
    // Sort the identities before formatting their Rust literals: Debug escaping changes their
    // lexical order (for example, `a\n` sorts before `a0` as an identity, but after it escaped).
    edges.sort_by(|left, right| {
        left.source_id
            .cmp(&right.source_id)
            .then_with(|| left.target_id.cmp(&right.target_id))
    });
    let edges = edges
        .into_iter()
        .map(|edge| format!("({:?}, {:?})", edge.source_id, edge.target_id))
        .collect::<Vec<_>>();
    format!(
        "{{ let edges: &[(&str, &str)] = &[{}]; let mut visited: Vec<&str> = Vec::new(); \
         let mut frontier = vec![{:?}]; while let Some(current) = frontier.pop() {{ \
         if visited.iter().any(|known| *known == current) {{ continue; }} \
         if visited.len() == {} {{ return false; }} visited.push(current); \
         for (source, target) in edges.iter().rev() {{ if *source == current {{ \
         if *target == {:?} {{ return true; }} frontier.push(*target); }} }} }} false }}",
        edges.join(", "),
        lowered.request.start_id,
        lowered.request.max_expansions,
        lowered.request.target_id,
    )
}

/// Renders one corpus case's artifacts, all named from the case's `name`. The only failure is the
/// serialization of the proof-dependency graph, which [`generate_bounded_kani_corpus_case`]
/// refuses as `kani_corpus_serialization_failed`.
///
/// `dependencies` is the already-validated, already-normalized declared census (see
/// [`normalize_dependencies`]).
fn render_artifacts(
    ceilings: crate::kani::identity::ProofCeilings,
    family: BoundedCorpusFamily,
    name: &str,
    value: bool,
    oracle_body: &str,
    qsl_body: &str,
    dependencies: &[ProofDependencyEdge],
) -> Result<BoundedCorpusArtifacts, String> {
    let label = family.label();
    let value_literal = if value { "true" } else { "false" };
    let mut oracle = String::new();
    let _ = writeln!(oracle, "// Generated bounded-Kani {label} oracle: {name}");
    let _ = writeln!(oracle, "pub fn corpus_oracle() -> bool {{ {oracle_body} }}");
    let mut strategy = String::new();
    let _ = writeln!(strategy, "// Generated finite strategy: {name}");
    let _ = writeln!(strategy, "pub const CORPUS_CASE: bool = {value_literal};");
    // The proof symbol carries the same case name the artifact paths carry, so two cases never
    // declare one `#[kani::proof]` symbol. The identical string is `proof_graph.proof_id` below,
    // so the graph names the exact symbol it describes.
    let proof_id = format!("corpus_case_{name}");
    let mut harness = String::new();
    let _ = writeln!(harness, "// Generated Kani harness: {name}");
    let _ = writeln!(harness, "#[kani::proof]");
    let _ = writeln!(harness, "fn {proof_id}() {{");
    let _ = writeln!(harness, "    assert!(corpus_oracle());");
    let _ = writeln!(
        harness,
        "    kani::cover!(true, \"the harness runs to its end past the oracle assertion\");"
    );
    let _ = writeln!(harness, "}}");
    let proof_graph_value = CorpusProofDependencyGraph {
        ceilings,
        schema_version: CORPUS_PROOF_GRAPH_SCHEMA.to_owned(),
        proof_id,
        family: label.to_owned(),
        construct: family.construct().to_owned(),
        identity: name.to_owned(),
        readiness: dependency_readiness(dependencies),
        dependencies: dependencies.to_vec(),
    };
    let proof_graph_contents =
        artifact_json(&proof_graph_value).map_err(|error| error.to_string())?;
    let qsl_source = String::from(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile v = \"quire.value.complete/v1\";\n\
         import \"ir-453/corpus-dependency\" as d;\n\
         function corpus_case using v(): Boolean pure { d::corpus_case(true) }\n",
    );
    let qsl_functions = if qsl_body.starts_with("function edge ") {
        qsl_body.to_owned()
    } else {
        format!(
            "function corpus_case using v(enabled: Boolean): Boolean pure {{ enabled and ({qsl_body}) }}"
        )
    };
    let qsl_dependency = format!(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile v = \"quire.value.complete/v1\";\n\
         {qsl_functions}\n"
    );
    Ok(BoundedCorpusArtifacts {
        oracle: Artifact::new(format!("corpus/{name}.oracle.rs"), oracle),
        strategy: Artifact::new(format!("corpus/{name}.strategy.rs"), strategy),
        kani_harness: Artifact::new(format!("corpus/{name}.kani.rs"), harness),
        qsl_source: Artifact::new(format!("corpus/{name}.qsl"), qsl_source),
        qsl_dependency: Artifact::new(format!("corpus/{name}.dependency.qsl"), qsl_dependency),
        proof_graph: Artifact::new(
            format!("corpus/{name}.proof-graph.json"),
            proof_graph_contents,
        ),
    })
}

fn compile_qsl_package(
    case: &BoundedCorpusCase,
) -> Result<crate::replay::function::ReplayPackage, crate::replay::function::ReplayPackageError> {
    use crate::replay::function::{DependencyLock, LockedSource, ReplayInputs, ReplayPackage};
    use qsl_replay::{
        call_site, DependencyInput, Identifier, QualifiedName, ReplayLimits, ScalarLimits,
        SourceIdentity,
    };
    use std::collections::BTreeMap;

    let maximum = u64::MAX;
    let dependency_source = LockedSource {
        authority: "agent-ix".to_owned(),
        identity: "ir-453/corpus-dependency".to_owned(),
        namespace: "generated".to_owned(),
        revision: "ir-453".to_owned(),
        bytes: case.artifacts.qsl_dependency.contents.as_bytes().to_vec(),
    };
    let function = "corpus_case";
    let invalid_function = || crate::replay::function::ReplayPackageError::InvalidFunction {
        function: function.to_owned(),
    };
    let dependency_function = QualifiedName::new(vec![
        Identifier::new(function).map_err(|_| invalid_function())?
    ])
    .map_err(|_| invalid_function())?;
    let dependency_package_id = call_site(
        SourceIdentity::new(
            &dependency_source.authority,
            &dependency_source.identity,
            &dependency_source.namespace,
            &dependency_source.revision,
        ),
        &dependency_source.identity,
        &dependency_source.bytes,
        [],
        &DependencyInput::default(),
        &dependency_function,
    )
    .map_err(crate::replay::function::ReplayPackageError::CallSite)?
    .package_id;
    ReplayPackage::new(
        ReplayInputs {
            source: LockedSource {
                authority: "agent-ix".to_owned(),
                identity: case.artifacts.qsl_source.path.clone(),
                namespace: "generated".to_owned(),
                revision: "ir-453".to_owned(),
                bytes: case.artifacts.qsl_source.contents.as_bytes().to_vec(),
            },
            dependencies: vec![DependencyLock {
                identity: dependency_source.identity.clone(),
                package_id: dependency_package_id,
                source: dependency_source,
            }],
            accounting_limits: ScalarLimits {
                integer_bits: maximum,
                decimal_digits: maximum,
                scale_expansion: maximum,
                text_input_bytes: maximum,
                text_scalars: maximum,
                normalized_scalars: maximum,
                unit_edges: maximum,
                value_occurrences: maximum,
                work_units: maximum,
                result_units: maximum,
            },
            stage_limits: BTreeMap::new(),
            declared_domains: Vec::new(),
            replay_limits: ReplayLimits::default(),
        },
        "corpus_case",
    )
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
        FiniteReference, KaniOutcome, KaniOutcomeKind, KaniProfile, ModuleDescriptor,
        PopulationCompleteness, ProfileSelection, ResourceBounds, SemanticFamily,
    };
    use quire_contract_model::{std001_code, NumericOperator, Std001Code};

    use super::{
        generate_bounded_kani_corpus_case, BoundedCorpusRequest, CorpusProofDependencyGraph,
        EmittedCorpusIdentities, ProofReadiness, CORPUS_PROOF_GRAPH_SCHEMA,
    };
    use super::{CheckedArithmeticRequest, CollectionQuery, GraphRequest, QueryKind};
    use crate::kani::census::{ProofDependencyKind, ProofDependencyRequest, ProofDependencyState};
    use crate::replay::function::ReplayPackage;
    use std::{
        fs,
        path::PathBuf,
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };

    type InputEdit = Box<dyn Fn(&mut FiniteInput)>;

    type Fixture = (
        KaniProfile,
        DispatchIndex,
        quire_contract_ir::kani::ValidatedFiniteInput,
    );

    fn qsl_package(case: &super::BoundedCorpusCase) -> ReplayPackage {
        super::compile_qsl_package(case).unwrap_or_else(|error| {
            panic!(
                "the case's QSL source compiles and selects its generated function: {error}\n{}",
                case.artifacts.qsl_source.contents
            )
        })
    }

    fn false_cases() -> Vec<super::BoundedCorpusCase> {
        let fixture = fixture();
        vec![
            emit(
                &fixture,
                BoundedCorpusRequest::Graph(GraphRequest {
                    source_id: "source".to_owned(),
                    start_id: "b".to_owned(),
                    target_id: "a".to_owned(),
                    field_id: "next".to_owned(),
                    max_expansions: 2,
                }),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap(),
            emit(
                &fixture,
                BoundedCorpusRequest::Collection(CollectionQuery {
                    source_id: "source".to_owned(),
                    values: vec![2, 2],
                    max_items: 2,
                    kind: QueryKind::ExistsEqual(7),
                }),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap(),
        ]
    }

    struct KaniTemp(PathBuf);

    impl KaniTemp {
        fn new(case: &super::BoundedCorpusCase) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock is after Unix epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!("ir-453-{}-{nonce}", std::process::id()));
            fs::create_dir_all(path.join("src")).expect("temp crate source directory exists");
            fs::write(
                path.join("src/lib.rs"),
                format!(
                    "{}\n{}",
                    case.artifacts.oracle.contents, case.artifacts.kani_harness.contents
                ),
            )
            .expect("corpus Rust sources are writable");
            fs::write(
                path.join("Cargo.toml"),
                "[package]\nname = \"ir453-corpus-playback\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
            )
            .expect("temporary manifest is writable");
            fs::write(
                path.join("build.rs"),
                "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
            )
            .expect("temporary build script is writable");
            Self(path)
        }

        fn run(&self, case: &super::BoundedCorpusCase) -> crate::kani::classify::KaniRunOutcome {
            let report_path = self.0.join("report.json");
            let output = Command::new("cargo")
                .args([
                    "kani",
                    "-Z",
                    "concrete-playback",
                    "-Z",
                    "unstable-options",
                    "--export-json",
                ])
                .arg(&report_path)
                .args([
                    "--harness",
                    case.harness_symbol.as_str(),
                    "--concrete-playback",
                    "print",
                ])
                .env(
                    "CARGO_TARGET_DIR",
                    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("target/ir-453-bounded-corpus-replay-kani"),
                )
                .current_dir(&self.0)
                .output()
                .expect("cargo kani launches");
            let text = format!(
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let report = fs::read(report_path).ok();
            crate::classify_kani_run(output.status.success(), report.as_deref(), &text, None)
                .unwrap_or_else(|refusal| panic!("Kani run refused as {refusal:?}: {text}"))
                .outcome
        }
    }

    impl Drop for KaniTemp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Each supported family retains source bytes that QSL compiles into the selected function
    /// used by replay. No case turns its generated verdict into a literal `ReplaySource::Input`.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_each_corpus_case_compiles_its_retained_qsl_source() {
        let (profile, dispatch, input) = fixture();
        let cases = vec![
            generate_bounded_kani_corpus_case(
                crate::kani::test_support::proof_ceilings::proof_ceilings(),
                &profile,
                &dispatch,
                &input,
                BoundedCorpusRequest::Arithmetic(CheckedArithmeticRequest {
                    source_id: "source",
                    operator: NumericOperator::Add,
                    left: 1,
                    right: 1,
                    minimum: 0,
                    maximum: 2,
                }),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap(),
            emit(
                &(profile.clone(), dispatch.clone(), input.clone()),
                BoundedCorpusRequest::Graph(GraphRequest {
                    source_id: "source".to_owned(),
                    start_id: "a".to_owned(),
                    target_id: "b".to_owned(),
                    field_id: "next".to_owned(),
                    max_expansions: 2,
                }),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap(),
            emit(
                &(profile.clone(), dispatch.clone(), input.clone()),
                BoundedCorpusRequest::Graph(GraphRequest {
                    source_id: "source".to_owned(),
                    start_id: "b".to_owned(),
                    target_id: "a".to_owned(),
                    field_id: "next".to_owned(),
                    max_expansions: 2,
                }),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap(),
            emit(
                &(profile.clone(), dispatch.clone(), input.clone()),
                BoundedCorpusRequest::Collection(CollectionQuery {
                    source_id: "source".to_owned(),
                    values: vec![2, 2, 7],
                    max_items: 3,
                    kind: QueryKind::ExistsEqual(7),
                }),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap(),
            emit(
                &(profile.clone(), dispatch.clone(), input.clone()),
                BoundedCorpusRequest::Collection(CollectionQuery {
                    source_id: "source".to_owned(),
                    values: vec![2, 2],
                    max_items: 2,
                    kind: QueryKind::ExistsEqual(7),
                }),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap(),
        ];
        for case in &cases {
            let package = qsl_package(case);
            let identity = package
                .harness_identity(None, case.harness_symbol.clone(), Vec::new())
                .expect("the zero-argument selected function has a typed identity");
            assert!(package.source_matches(case.artifacts.qsl_source.contents.as_bytes()));
            assert!(identity.matches_package(&package));
            assert!(package.parameters().is_empty());
            assert_eq!(
                package.evaluate_boolean_function(&identity).unwrap(),
                case.outcome
                    .boolean_claim()
                    .expect("admitted case has a Boolean claim"),
                "QSL evaluates the retained source to the case's semantic result"
            );
        }
        let first = qsl_package(&cases[0]);
        let tied_identity = first
            .harness_identity(None, cases[0].harness_symbol.clone(), Vec::new())
            .unwrap();
        let other_package = qsl_package(&cases[1]);
        assert!(!tied_identity.matches_package(&other_package));
        assert_ne!(
            cases[0].artifacts.qsl_dependency, cases[1].artifacts.qsl_dependency,
            "each imported dependency carries the case's own semantic source"
        );
    }

    /// The two false corpus families require real Kani playback and a reproduced typed QSL
    /// violation. These are deliberately serial, ignored Kani-lane tests.
    ///
    /// Trace: TC-023, FR-015-AC-57.
    #[test]
    #[ignore = "kani lane: run serially after the IR-453 focused checks"]
    fn tc_023_real_corpus_playback_replays_through_its_qsl_source() {
        use crate::kani::classify::KaniRunOutcome;
        use crate::replay::function::replay_harness_counterexample;
        use crate::{EvidenceFailureCause, HarnessSymbol, ReplayVerdict};

        let cases = false_cases();
        for case in &cases {
            let KaniRunOutcome::Falsified { counterexample } = KaniTemp::new(&case).run(&case)
            else {
                panic!("the generated false case must be falsified by Kani")
            };
            let verdict = case
                .replay_counterexample(&counterexample)
                .expect("QSL accepts the source-backed replay request");
            assert_eq!(verdict, ReplayVerdict::Reproduced);
            let package = qsl_package(case);
            let identity = package
                .harness_identity(None, case.harness_symbol.clone(), Vec::new())
                .expect("selected QSL function has a typed source identity");

            let other_case = cases
                .iter()
                .find(|other| other.artifacts.qsl_source != case.artifacts.qsl_source)
                .expect("the graph and collection cases retain different QSL sources");
            let wrong_package = qsl_package(other_case);
            assert_eq!(
                replay_harness_counterexample(&identity, &counterexample, &wrong_package)
                    .expect("a package mismatch is an evidence verdict"),
                ReplayVerdict::EvidenceFailure(EvidenceFailureCause::PackageIdentity)
            );

            let wrong_harness = package
                .harness_identity(
                    None,
                    HarnessSymbol::try_from("another_harness").unwrap(),
                    Vec::new(),
                )
                .unwrap();
            assert!(matches!(
                replay_harness_counterexample(&wrong_harness, &counterexample, &package),
                Ok(ReplayVerdict::EvidenceFailure(
                    EvidenceFailureCause::Decode(_)
                ))
            ));
        }
    }

    /// Trace: TC-023. Owner serializers supply each value; the input projection
    /// retains three established key names and the two intentionally normalized populations.
    #[test]
    fn tc_023_canonical_preimage_uses_owner_serializers() {
        let (profile, _, input) = fixture();
        let ceilings = crate::kani::test_support::proof_ceilings::proof_ceilings();
        let normalized = super::normalized_input_value(input.input()).unwrap();
        let owner = serde_json::to_value(input.input()).unwrap();
        for (field, value) in owner.as_object().unwrap() {
            let identity_field = match field.as_str() {
                "input_bytes" => "inputBytes",
                "model_id" => "modelId",
                "source_id" => "sourceId",
                "objects" | "references" => continue,
                other => other,
            };
            assert_eq!(normalized.get(identity_field), Some(value), "{field}");
        }
        assert!(normalized.get("input_bytes").is_none());
        assert!(normalized.get("model_id").is_none());
        assert!(normalized.get("source_id").is_none());
        assert_eq!(
            normalized["objects"],
            serde_json::json!([["a", "node", "s"], ["b", "node", "s"]])
        );
        assert_eq!(
            normalized["references"],
            serde_json::json!([["a", "next", "b"]])
        );
        let edge = crate::kani::census::ProofDependencyEdge {
            proof_id: "dependency".to_owned(),
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            source_site: None,
        };
        let dependencies = [edge];
        let request = arithmetic("source", 1, 1);
        let identity = super::CaseIdentity {
            ceilings,
            construct: "checked-arithmetic",
            profile: profile.selection(),
            input: normalized,
            request: super::RequestIdentity::from(&request),
            dependencies: &dependencies,
        };
        let preimage = crate::core::canonical::preimage_value(&identity).unwrap();
        assert_eq!(
            preimage["ceilings"],
            serde_json::to_value(ceilings).unwrap()
        );
        assert_eq!(
            preimage["profile"],
            serde_json::to_value(profile.selection()).unwrap()
        );
        assert_eq!(
            preimage["dependencies"],
            serde_json::to_value(&dependencies).unwrap()
        );
    }

    /// Trace: TC-023. The owning input's object maps may exceed the final case-name
    /// ceiling even though their normalized tuples fit. Admission is decided by the final bytes.
    #[test]
    fn tc_023_compact_case_identity_admits_a_verbose_owner_input() {
        let fixture = fixture_with(
            |_| {},
            |input| {
                input.objects = (0..18_500)
                    .map(|index| FiniteObject {
                        identity: format!("{index:06}"),
                        type_id: "node".to_owned(),
                        snapshot_id: "s".to_owned(),
                    })
                    .collect();
                input.references.clear();
                input.bounds.max_objects = 18_500;
                input.bounds.max_references = 0;
                input.bounds.max_input_bytes = 2_000_000;
                input.input_bytes = 1_055_000;
            },
        );
        let owner = serde_json::to_vec(fixture.2.input()).unwrap();
        assert!(owner.len() > 1 << 20, "the old preflight would refuse");
        let normalized = super::normalized_input_value(fixture.2.input()).unwrap();
        assert!(
            serde_json::to_vec(&normalized).unwrap().len() < 1 << 20,
            "the compact identity input fits the final ceiling"
        );

        let case = emit(
            &fixture,
            arithmetic("source", 1, 1),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .expect("the bounded canonical case identity admits");
        assert!(case_name(&case).starts_with("arithmetic_"));
    }

    /// Trace: TC-023. This preimage is authored from the fixture fields in RFC 8785
    /// key order, not obtained from the generator or canonical encoder. The fixed digest pins the
    /// emitted name, so replacing the canonical digest with struct-order JSON changes this test.
    /// Serde's enum-level camelCase renames the variant, but leaves its struct-variant
    /// `source_id` field in snake_case.
    #[test]
    fn tc_023_case_name_matches_hand_authored_canonical_preimage() {
        const PREIMAGE: &str = concat!(
            r#"{"ceilings":{"memoryBytes":17179869184,"wallClock":{"nanos":0,"secs":600}},"#,
            r#""construct":"checked-arithmetic","dependencies":[],"input":{"bounds":{"#,
            r#""max_input_bytes":2,"max_objects":2,"max_references":1},"completeness":"complete","#,
            r#""inputBytes":1,"modelId":"model","objects":[["a","node","s"],["b","node","s"]],"#,
            r#""profile":{"abi_revision":"abi","profile":"kani-bounded/1","revision":"r1"},"#,
            r#""references":[["a","next","b"]],"sourceId":"source"},"#,
            r#""profile":{"abi_revision":"abi","profile":"kani-bounded/1","revision":"r1"},"#,
            r#""request":{"arithmetic":{"left":"1","maximum":"2","minimum":"0","#,
            r#""operator":"checked_add","right":"1","source_id":"source"}}}"#,
        );
        const DIGEST: &str = "4afb9c72a943cbd2db73a206e083012480ee4218adb534e96cbc13c44cc7a525";
        assert_eq!(
            format!("{:x}", qsl_replay::ByteDigest::of(PREIMAGE.as_bytes())),
            DIGEST,
            "the hand-authored preimage must retain its fixed digest"
        );
        let case = emit(
            &fixture(),
            arithmetic("source", 1, 1),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .expect("the fixture is an admitted arithmetic case");
        assert_eq!(case_name(&case), format!("arithmetic_{DIGEST}"));
    }

    /// Trace: TC-023. The caller supplies the dependency census, so it can offer a
    /// graph larger than one artifact. The counted preflight refuses before building a JSON tree.
    #[test]
    fn tc_023_large_dependency_graph_refuses_canonical_projection() {
        let dependencies = (0..20_000)
            .map(|index| crate::kani::census::ProofDependencyEdge {
                proof_id: format!("dep_{index:05}_{}", "x".repeat(1024)),
                kind: ProofDependencyKind::Required,
                state: ProofDependencyState::Passed,
                source_site: None,
            })
            .collect();
        let graph = CorpusProofDependencyGraph {
            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings(),
            schema_version: CORPUS_PROOF_GRAPH_SCHEMA.to_owned(),
            proof_id: "corpus_proof".to_owned(),
            family: "collection".to_owned(),
            construct: "bounded-collection-query".to_owned(),
            identity: "collection_case".to_owned(),
            readiness: ProofReadiness::Ready,
            dependencies,
        };
        let error = crate::core::canonical::artifact_json(&graph)
            .expect_err("the projection exceeds one artifact");
        assert!(matches!(
            error,
            crate::core::canonical::ArtifactJsonError::Projection(ref source)
                if source.is_io()
                    && source.to_string().contains("exceeds the artifact ceiling")
        ));
    }

    /// Trace: FR-028-AC-1.
    #[test]
    fn either_ceiling_changes_the_canonical_corpus_identity() {
        let (profile, dispatch, input) = fixture();
        let original = crate::kani::identity::ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(128 * 1024 * 1024).unwrap(),
            wall_clock: std::time::Duration::from_secs(30),
        };
        let generate = |ceilings| {
            generate_bounded_kani_corpus_case(
                ceilings,
                &profile,
                &dispatch,
                &input,
                arithmetic("source", 1, 1),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap()
        };
        let baseline = generate(original);
        for ceilings in [
            crate::kani::identity::ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(64 * 1024 * 1024).unwrap(),
                ..original
            },
            crate::kani::identity::ProofCeilings {
                wall_clock: std::time::Duration::from_secs(15),
                ..original
            },
        ] {
            let changed = generate(ceilings);
            assert_ne!(
                baseline.artifacts.kani_harness.path,
                changed.artifacts.kani_harness.path
            );
            let graph: CorpusProofDependencyGraph =
                serde_json::from_str(&changed.artifacts.proof_graph.contents).unwrap();
            assert_eq!(graph.ceilings, ceilings);
        }
    }

    fn fixture() -> Fixture {
        fixture_with(|_| {}, |_| {})
    }

    /// The standard fixture with its profile selection and finite input edited before validation.
    fn fixture_with(
        edit_selection: impl FnOnce(&mut ProfileSelection),
        edit_input: impl FnOnce(&mut FiniteInput),
    ) -> Fixture {
        let mut selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        edit_selection(&mut selection);
        let profile = KaniProfile::new(
            selection.clone(),
            vec![
                CapabilityEntry {
                    construct: "checked-arithmetic".to_owned(),
                    disposition: CapabilityDisposition::Supported {
                        module: "arithmetic".to_owned(),
                    },
                },
                CapabilityEntry {
                    construct: "finite-reference-graph".to_owned(),
                    disposition: CapabilityDisposition::Supported {
                        module: "graphs".to_owned(),
                    },
                },
                CapabilityEntry {
                    construct: "bounded-collection-query".to_owned(),
                    disposition: CapabilityDisposition::Supported {
                        module: "collections".to_owned(),
                    },
                },
            ],
        )
        .unwrap();
        let dispatch = DispatchIndex::new(vec![
            ModuleDescriptor {
                module_id: "arithmetic".to_owned(),
                family: SemanticFamily::DefinednessArithmetic,
                abi_revision: "abi".to_owned(),
                constructs: vec!["checked-arithmetic".to_owned()],
            },
            ModuleDescriptor {
                module_id: "graphs".to_owned(),
                family: SemanticFamily::ObjectsReferencesGraphs,
                abi_revision: "abi".to_owned(),
                constructs: vec!["finite-reference-graph".to_owned()],
            },
            ModuleDescriptor {
                module_id: "collections".to_owned(),
                family: SemanticFamily::CollectionsQueries,
                abi_revision: "abi".to_owned(),
                constructs: vec!["bounded-collection-query".to_owned()],
            },
        ])
        .unwrap();
        let mut input = FiniteInput {
            model_id: "model".to_owned(),
            source_id: "source".to_owned(),
            profile: selection,
            completeness: PopulationCompleteness::Complete,
            bounds: ResourceBounds {
                max_objects: 2,
                max_references: 1,
                max_input_bytes: 2,
            },
            input_bytes: 1,
            objects: vec![
                FiniteObject {
                    identity: "a".to_owned(),
                    type_id: "node".to_owned(),
                    snapshot_id: "s".to_owned(),
                },
                FiniteObject {
                    identity: "b".to_owned(),
                    type_id: "node".to_owned(),
                    snapshot_id: "s".to_owned(),
                },
            ],
            references: vec![FiniteReference {
                source_id: "a".to_owned(),
                field_id: "next".to_owned(),
                target_id: "b".to_owned(),
            }],
        };
        edit_input(&mut input);
        let input = input.validate().unwrap();
        (profile, dispatch, input)
    }

    fn arithmetic(source_id: &'static str, left: i128, right: i128) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Arithmetic(CheckedArithmeticRequest {
            source_id,
            operator: NumericOperator::Add,
            left,
            right,
            minimum: 0,
            maximum: 2,
        })
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_generates_deterministic_complete_artifacts_for_every_supported_family() {
        let (profile, dispatch, input) = fixture();
        let cases = vec![
            arithmetic("source", 1, 1),
            BoundedCorpusRequest::Graph(GraphRequest {
                source_id: "source".to_owned(),
                start_id: "a".to_owned(),
                target_id: "b".to_owned(),
                field_id: "next".to_owned(),
                max_expansions: 2,
            }),
            BoundedCorpusRequest::Collection(CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(7),
            }),
        ];
        for request in cases {
            // A fresh counter per call: an identical request yields identical output.
            let first = generate_bounded_kani_corpus_case(
                crate::kani::test_support::proof_ceilings::proof_ceilings(),
                &profile,
                &dispatch,
                &input,
                request.clone(),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap();
            let second = generate_bounded_kani_corpus_case(
                crate::kani::test_support::proof_ceilings::proof_ceilings(),
                &profile,
                &dispatch,
                &input,
                request,
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap();
            assert_eq!(first, second);
            assert_eq!(first.outcome.boolean_claim(), Some(true));
            assert!(!first
                .artifacts
                .kani_harness
                .contents
                .contains("kani::assume"));
            let graph: CorpusProofDependencyGraph =
                serde_json::from_str(&first.artifacts.proof_graph.contents)
                    .expect("the corpus proof-dependency graph must deserialize");
            assert_eq!(graph.schema_version, CORPUS_PROOF_GRAPH_SCHEMA);
            assert!(graph.dependencies.is_empty());
            assert_eq!(graph.readiness, ProofReadiness::Ready);
            assert_eq!(
                graph.proof_id,
                proof_symbol(&first.artifacts.kani_harness.contents),
                "the graph's proof id must equal the harness's own #[kani::proof] symbol"
            );
            syn::parse_file(&format!(
                "{}\n{}",
                first.artifacts.oracle.contents, first.artifacts.kani_harness.contents
            ))
            .expect("the generated oracle and Kani harness must be valid Rust syntax");
        }
    }

    /// A declared `Required` dependency must actually reach the emitted graph, not be silently
    /// dropped (ir#80).
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_declared_required_dependency_appears_in_the_graph() {
        let (profile, dispatch, input) = fixture();
        let dependency = ProofDependencyRequest {
            proof_id: "upstream-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        };
        let with_dependency = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("with-dependency", 1, 1),
            &[dependency],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        let graph: CorpusProofDependencyGraph =
            serde_json::from_str(&with_dependency.artifacts.proof_graph.contents)
                .expect("the corpus proof-dependency graph must deserialize");
        assert_eq!(graph.dependencies.len(), 1);
        assert_eq!(graph.dependencies[0].proof_id, "upstream-lemma");
        assert_eq!(graph.dependencies[0].kind, ProofDependencyKind::Required);
        assert_eq!(graph.dependencies[0].state, ProofDependencyState::Passed);
        assert_eq!(graph.dependencies[0].source_site, None);
        assert_eq!(graph.readiness, ProofReadiness::Ready);
    }

    /// A declared `Required` dependency whose state is not `Passed` makes the case `Incomplete`.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_missing_required_dependency_yields_incomplete_readiness() {
        let (profile, dispatch, input) = fixture();
        let dependency = ProofDependencyRequest {
            proof_id: "missing-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Missing,
            original_path: None,
            replacement_path: None,
        };
        let generated = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("missing-dependency", 1, 1),
            &[dependency],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        let graph: CorpusProofDependencyGraph =
            serde_json::from_str(&generated.artifacts.proof_graph.contents)
                .expect("the corpus proof-dependency graph must deserialize");
        assert_eq!(graph.readiness, ProofReadiness::Incomplete);
    }

    /// A declared census with a duplicate proof identity is refused by the shared dependency
    /// rules, and the refusal claims no identity: the same request with a valid census is accepted.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_duplicate_dependency_identity_is_refused_and_claims_no_identity() {
        let (profile, dispatch, input) = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let duplicate = ProofDependencyRequest {
            proof_id: "upstream-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        };
        let error = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("duplicate-census", 1, 1),
            &[duplicate, duplicate],
            &mut emitted,
        )
        .map_err(refusal_outcome)
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(error.code, std001_code!("kani_corpus_dependency_invalid"));
        let retry = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("duplicate-census", 1, 1),
            &[],
            &mut emitted,
        )
        .unwrap();
        assert_eq!(
            retry.artifacts.oracle.path,
            format!("corpus/{}.oracle.rs", case_name(&retry))
        );
    }

    /// A finite input validated under another profile selection than the profile passed in is
    /// refused as `InvalidInput` `kani_profile_input_mismatch`, naming the request's source and
    /// the profile's own revision, before any artifact is emitted and without claiming an identity.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_an_input_for_another_profile_selection_is_refused_before_any_artifact() {
        let (profile, dispatch, _) = fixture();
        let (_, _, other_input) =
            fixture_with(|selection| selection.revision = "r2".to_owned(), |_| {});
        let mut emitted = EmittedCorpusIdentities::new();
        let error = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &other_input,
            arithmetic("mismatched-profile", 1, 1),
            &[],
            &mut emitted,
        )
        .map_err(refusal_outcome)
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(error.code, std001_code!("kani_profile_input_mismatch"));
        assert_eq!(error.source_id, "mismatched-profile");
        assert_eq!(error.context, "r1");
        assert!(
            emitted.0.is_empty(),
            "the refused case must claim no identity in the registry"
        );
    }

    /// The profile's revision is the context of every outcome the corpus returns: a proved case,
    /// a counterexample, and an invalid census refusal.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_outcomes_and_refusals_carry_the_profile_revision_as_context() {
        let (profile, dispatch, input) =
            fixture_with(|selection| selection.revision = "rev-7".to_owned(), |_| {});
        let proved = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("proved-case", 1, 1),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert_eq!(proved.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(proved.outcome.context, "rev-7");
        let falsified = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Collection(CollectionQuery {
                source_id: "counterexample-case".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(8),
            }),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert_eq!(falsified.outcome.kind, KaniOutcomeKind::Counterexample);
        assert_eq!(falsified.outcome.context, "rev-7");
        let duplicate = ProofDependencyRequest {
            proof_id: "upstream-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        };
        let refused = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("refused-case", 1, 1),
            &[duplicate, duplicate],
            &mut EmittedCorpusIdentities::new(),
        )
        .map_err(refusal_outcome)
        .unwrap_err();
        assert_eq!(refused.code, std001_code!("kani_corpus_dependency_invalid"));
        assert_eq!(refused.context, "rev-7");
    }

    /// This corpus's generated harnesses render no `// proof-dependency-site:` marker, no
    /// `kani::assume`, and no `#[kani::stub]`, so a declared `Assumed` dependency must be refused
    /// (ir#80 review finding F1), claiming no identity.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_assumed_dependency_kind_is_refused_and_claims_no_identity() {
        let (profile, dispatch, input) = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let assumed = ProofDependencyRequest {
            proof_id: "assumed-lemma",
            kind: ProofDependencyKind::Assumed,
            state: ProofDependencyState::Assumed,
            original_path: Some("crate::assumed_predicate"),
            replacement_path: None,
        };
        let error = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("assumed-census", 1, 1),
            &[assumed],
            &mut emitted,
        )
        .map_err(refusal_outcome)
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(error.code, std001_code!("kani_corpus_dependency_invalid"));
        let retry = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            arithmetic("assumed-census", 1, 1),
            &[],
            &mut emitted,
        )
        .unwrap();
        assert_eq!(
            retry.artifacts.oracle.path,
            format!("corpus/{}.oracle.rs", case_name(&retry))
        );
    }

    /// Contract IR's refusal to build a non-success outcome for a `proved` request is carried as
    /// the typed `OutcomeConstruction` refusal with IR's own code, and holds no outcome. The
    /// generator never makes that request, so this is the only way to reach the variant.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_an_outcome_construction_refusal_is_typed_and_carries_no_outcome() {
        let error = KaniOutcome::non_success(
            KaniOutcomeKind::Proved,
            std001_code!("kani_corpus_identity_collision"),
            "source",
            "r1",
        )
        .map_err(super::BoundedCorpusError::from)
        .unwrap_err();
        assert_eq!(error.code(), Std001Code::KANI_OUTCOME_INVALID);
        assert_eq!(error.outcome(), None);
        assert!(
            std::error::Error::source(&error).is_some(),
            "the wrapped KaniOutcomeError is the error's source"
        );
        assert!(error.to_string().starts_with("kani_outcome_invalid"));
        let refusal = super::BoundedCorpusError::from(
            KaniOutcome::non_success(
                KaniOutcomeKind::Refused,
                std001_code!("kani_corpus_serialization_failed"),
                "case-7",
                "rev-9",
            )
            .expect("a Refused non-success is buildable"),
        );
        assert!(std::error::Error::source(&refusal).is_none());
        assert_eq!(
            refusal.to_string(),
            "kani_corpus_serialization_failed: Refused for source \"case-7\" in context \"rev-9\""
        );
        let outcome = KaniOutcome::counterexample("source", "r1");
        assert_eq!(
            super::BoundedCorpusError::from(outcome.clone()).outcome(),
            Some(&outcome)
        );
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_non_success_emits_no_partial_artifacts_or_boolean_claim() {
        let (profile, dispatch, input) = fixture();
        let error = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Collection(CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![1, 2],
                max_items: 1,
                kind: QueryKind::ForAllNonNegative,
            }),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .map_err(refusal_outcome)
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::ResourceExhausted);
        assert_eq!(error.boolean_claim(), None);
        assert_eq!(
            error.context, "r1",
            "a lowering refusal returned through the corpus carries the profile revision"
        );
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_unreachable_graph_request_classifies_as_false() {
        let (profile, dispatch, input) = fixture();
        let generated = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Graph(GraphRequest {
                source_id: "source".to_owned(),
                start_id: "b".to_owned(),
                target_id: "a".to_owned(),
                field_id: "next".to_owned(),
                max_expansions: 2,
            }),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Counterexample);
        assert_eq!(generated.outcome.boolean_claim(), Some(false));
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_admitted_zero_arithmetic_is_a_proof_not_a_false_verdict() {
        let (profile, dispatch, input) = fixture();
        let generated = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(CheckedArithmeticRequest {
                source_id: "checked-zero",
                operator: NumericOperator::Subtract,
                left: 1,
                right: 1,
                minimum: 0,
                maximum: 1,
            }),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(generated.outcome.source_id, "checked-zero");
    }

    /// A provable request with an operand outside `i64`'s range still generates.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_provable_arithmetic_with_an_out_of_i64_range_operand_still_generates() {
        let (profile, dispatch, input) = fixture();
        let left = i64::MAX as i128 + 1;
        let generated = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(CheckedArithmeticRequest {
                source_id: "out-of-i64-range",
                operator: NumericOperator::Add,
                left,
                right: 0,
                minimum: left,
                maximum: left,
            }),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .expect("a provable case with an operand outside i64's range generates");
        assert_eq!(generated.outcome.kind, KaniOutcomeKind::Proved);
        assert_eq!(generated.outcome.source_id, "out-of-i64-range");
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_collection_oracle_evaluates_the_selected_ordered_population() {
        let (profile, dispatch, input) = fixture();
        let generated = generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Collection(CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(7),
            }),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert!(generated
            .artifacts
            .oracle
            .contents
            .contains("let values = [2i128, 2i128, 7i128]"));
        assert!(generated
            .artifacts
            .oracle
            .contents
            .contains("values.iter().any(|value| *value == 7i128)"));
    }

    /// The case name carried by a case's own harness path.
    fn case_name(case: &super::BoundedCorpusCase) -> String {
        case.artifacts
            .kani_harness
            .path
            .strip_prefix("corpus/")
            .and_then(|path| path.strip_suffix(".kani.rs"))
            .expect("harness path is corpus/<name>.kani.rs")
            .to_owned()
    }

    fn emit(
        fixture: &Fixture,
        request: BoundedCorpusRequest,
        dependencies: &[ProofDependencyRequest<'_>],
        emitted: &mut EmittedCorpusIdentities,
    ) -> Result<super::BoundedCorpusCase, super::BoundedCorpusError> {
        let (profile, dispatch, input) = fixture;
        generate_bounded_kani_corpus_case(
            crate::kani::test_support::proof_ceilings::proof_ceilings(),
            profile,
            dispatch,
            input,
            request,
            dependencies,
            emitted,
        )
    }

    /// The typed outcome of a refusal. A refusal that carries no outcome fails the test, which
    /// is what a request this module never makes (a `proved` or `counterexample` non-success)
    /// would be.
    fn refusal_outcome(error: super::BoundedCorpusError) -> quire_contract_ir::kani::KaniOutcome {
        match error {
            super::BoundedCorpusError::Outcome(outcome) => outcome,
            other @ super::BoundedCorpusError::OutcomeConstruction(_) => {
                panic!("expected a typed refusal outcome, got {other}")
            }
        }
    }

    fn collection(max_items: usize, source_id: &str) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Collection(CollectionQuery {
            source_id: source_id.to_owned(),
            values: vec![2, 2, 7],
            max_items,
            kind: QueryKind::ExistsEqual(7),
        })
    }

    /// Trace: TC-023, NFR-005-AC-1. The canonical preimage keeps both adjacent wide i128
    /// values exactly by spelling them as decimal strings, so the identity never rounds them.
    #[test]
    fn tc_023_wide_request_values_are_exact_canonical_strings() {
        let first_request = arithmetic("wide", (1_i128 << 53) + 1, 0);
        let second_request = arithmetic("wide", (1_i128 << 53) + 2, 0);
        let first = super::RequestIdentity::from(&first_request);
        let second = super::RequestIdentity::from(&second_request);
        let first_bytes = crate::core::canonical::content_bytes(&first).expect("string preimage");
        let second_bytes = crate::core::canonical::content_bytes(&second).expect("string preimage");
        assert_ne!(first_bytes, second_bytes);
        assert!(first_bytes
            .windows(b"9007199254740993".len())
            .any(|part| part == b"9007199254740993"));
        assert!(second_bytes
            .windows(b"9007199254740994".len())
            .any(|part| part == b"9007199254740994"));
    }

    /// Trace: TC-023, NFR-005-AC-1. An over-bound canonical preimage returns the typed input
    /// refusal before lowering or claiming a registry identity; a valid retry still admits.
    #[test]
    fn tc_023_unencodable_case_identity_refuses_without_claiming() {
        let fixture = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let source_id = "x".repeat(1 << 20);
        let refusal = refusal_outcome(
            emit(&fixture, collection(3, &source_id), &[], &mut emitted)
                .expect_err("oversized preimage must refuse"),
        );
        assert_eq!(refusal.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(
            refusal.code,
            std001_code!("kani_corpus_identity_unencodable")
        );
        assert!(emitted.0.is_empty());
        emit(&fixture, collection(3, "source"), &[], &mut emitted)
            .expect("a later valid case still admits");
        assert_eq!(emitted.0.len(), 1);
    }

    /// A case's name, paths and `#[kani::proof]` symbol come from its request content, so the
    /// same request yields the same ones in any emission order and in any run, and the symbol
    /// carries the name its artifact paths carry.
    ///
    /// Trace: FR-015-AC-94, TC-023.
    #[test]
    fn tc_023_case_identity_is_independent_of_emission_order_and_run() {
        let fixture = fixture();
        let requests = || {
            [
                arithmetic("first", 1, 1),
                collection(3, "second"),
                arithmetic("third", 1, 0),
            ]
        };
        let mut forward = EmittedCorpusIdentities::new();
        let in_order: Vec<_> = requests()
            .into_iter()
            .map(|request| emit(&fixture, request, &[], &mut forward).unwrap())
            .collect();
        let mut backward = EmittedCorpusIdentities::new();
        let mut reversed: Vec<_> = requests()
            .into_iter()
            .rev()
            .map(|request| emit(&fixture, request, &[], &mut backward).unwrap())
            .collect();
        reversed.reverse();
        assert_eq!(in_order, reversed);
        for case in &in_order {
            assert_eq!(
                proof_symbol(&case.artifacts.kani_harness.contents),
                format!("corpus_case_{}", case_name(case)),
            );
        }
    }

    /// Requests that differ in any one field -- including a field the rendered oracle never
    /// reads (`max_items`), the source id, or the declared census -- get distinct names, paths
    /// and proof symbols, so no case overwrites another.
    ///
    /// Trace: FR-015-AC-94, TC-023.
    #[test]
    fn tc_023_distinct_requests_get_distinct_names_paths_and_proof_symbols() {
        let fixture = fixture();
        let census = [ProofDependencyRequest {
            proof_id: "upstream-lemma",
            kind: ProofDependencyKind::Required,
            state: ProofDependencyState::Passed,
            original_path: None,
            replacement_path: None,
        }];
        let mut emitted = EmittedCorpusIdentities::new();
        let cases = [
            emit(&fixture, arithmetic("a", 1, 1), &[], &mut emitted),
            emit(&fixture, arithmetic("b", 1, 1), &[], &mut emitted),
            emit(&fixture, arithmetic("a", 1, 0), &[], &mut emitted),
            emit(&fixture, arithmetic("a", 1, 1), &census, &mut emitted),
            emit(&fixture, collection(3, "a"), &[], &mut emitted),
            emit(&fixture, collection(4, "a"), &[], &mut emitted),
        ]
        .map(|case| case.expect("each distinct request is accepted"));
        let mut names: Vec<_> = cases.iter().map(case_name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), cases.len());
        let mut symbols: Vec<_> = cases
            .iter()
            .map(|case| proof_symbol(&case.artifacts.kani_harness.contents).to_owned())
            .collect();
        symbols.sort();
        symbols.dedup();
        assert_eq!(symbols.len(), cases.len());
    }

    /// Emitting one request twice through one registry refuses the second deterministically and
    /// leaves the first untouched; a fresh registry accepts it again with the same artifacts.
    ///
    /// Trace: TC-023.
    #[test]
    fn tc_023_a_request_emitted_twice_is_refused_as_an_identity_collision() {
        let fixture = fixture();
        let mut emitted = EmittedCorpusIdentities::new();
        let first = emit(&fixture, arithmetic("same", 1, 1), &[], &mut emitted).unwrap();
        for _ in 0..2 {
            let refusal = emit(&fixture, arithmetic("same", 1, 1), &[], &mut emitted)
                .map_err(refusal_outcome)
                .unwrap_err();
            assert_eq!(refusal.kind, KaniOutcomeKind::InvalidInput);
            assert_eq!(refusal.code, std001_code!("kani_corpus_identity_collision"));
            assert_eq!(refusal.context, "r1");
        }
        let again = emit(
            &fixture,
            arithmetic("same", 1, 1),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap();
        assert_eq!(first, again);
    }

    fn identity_of(fixture: &Fixture, request: BoundedCorpusRequest) -> String {
        case_name(&emit(fixture, request, &[], &mut EmittedCorpusIdentities::new()).unwrap())
    }

    /// Asserts that the base case and every variant each have a different identity.
    fn assert_each_variation_changes_identity(
        fixture: &Fixture,
        base: BoundedCorpusRequest,
        variants: Vec<(&str, BoundedCorpusRequest)>,
    ) {
        let base_identity = identity_of(fixture, base);
        let mut seen = vec![base_identity];
        for (field, request) in variants {
            let identity = identity_of(fixture, request);
            assert!(
                !seen.contains(&identity),
                "varying `{field}` alone must change the case identity"
            );
            seen.push(identity);
        }
    }

    fn checked(
        source_id: &'static str,
        operator: NumericOperator,
        [left, right, minimum, maximum]: [i128; 4],
    ) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Arithmetic(CheckedArithmeticRequest {
            source_id,
            operator,
            left,
            right,
            minimum,
            maximum,
        })
    }

    fn reach(
        source_id: &str,
        start_id: &str,
        target_id: &str,
        field_id: &str,
        max_expansions: usize,
    ) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Graph(GraphRequest {
            source_id: source_id.to_owned(),
            start_id: start_id.to_owned(),
            target_id: target_id.to_owned(),
            field_id: field_id.to_owned(),
            max_expansions,
        })
    }

    /// Trace: FR-015-AC-94, TC-023.
    #[test]
    fn tc_023_every_arithmetic_request_field_changes_the_identity() {
        use NumericOperator::{Add, Subtract};
        assert_each_variation_changes_identity(
            &fixture(),
            checked("s", Add, [1, 1, 0, 2]),
            vec![
                ("source_id", checked("t", Add, [1, 1, 0, 2])),
                ("operator", checked("s", Subtract, [1, 1, 0, 2])),
                ("left", checked("s", Add, [0, 1, 0, 2])),
                ("right", checked("s", Add, [1, 0, 0, 2])),
                ("minimum", checked("s", Add, [1, 1, -1, 2])),
                ("maximum", checked("s", Add, [1, 1, 0, 3])),
            ],
        );
    }

    /// Trace: FR-015-AC-94, TC-023.
    #[test]
    fn tc_023_every_graph_request_field_changes_the_identity() {
        assert_each_variation_changes_identity(
            &fixture(),
            reach("s", "a", "b", "next", 2),
            vec![
                ("source_id", reach("t", "a", "b", "next", 2)),
                ("start_id", reach("s", "b", "b", "next", 2)),
                ("target_id", reach("s", "a", "a", "next", 2)),
                ("field_id", reach("s", "a", "b", "other", 2)),
                ("max_expansions", reach("s", "a", "b", "next", 3)),
            ],
        );
    }

    /// Trace: FR-015-AC-94, TC-023.
    #[test]
    fn tc_023_every_collection_request_field_changes_the_identity() {
        let query = |source_id: &str, values: Vec<i128>, max_items, kind| {
            BoundedCorpusRequest::Collection(CollectionQuery {
                source_id: source_id.to_owned(),
                values,
                max_items,
                kind,
            })
        };
        assert_each_variation_changes_identity(
            &fixture(),
            query("s", vec![2, 2, 7], 3, QueryKind::ExistsEqual(7)),
            vec![
                (
                    "source_id",
                    query("t", vec![2, 2, 7], 3, QueryKind::ExistsEqual(7)),
                ),
                (
                    "values",
                    query("s", vec![2, 7, 7], 3, QueryKind::ExistsEqual(7)),
                ),
                (
                    "max_items",
                    query("s", vec![2, 2, 7], 4, QueryKind::ExistsEqual(7)),
                ),
                (
                    "kind",
                    query("s", vec![2, 2, 7], 3, QueryKind::ForAllNonNegative),
                ),
                (
                    "expected value",
                    query("s", vec![2, 2, 7], 3, QueryKind::ExistsEqual(2)),
                ),
            ],
        );
    }

    /// Every part of the finite input and the profile selection is part of the identity: the
    /// graph oracle is built from the input's references, and the input also fixes the bounds
    /// the case was admitted under.
    ///
    /// Trace: FR-015-AC-94, TC-023.
    #[test]
    fn tc_023_every_input_and_profile_field_changes_the_identity() {
        let request = || checked("s", NumericOperator::Add, [1, 1, 0, 2]);
        let base = identity_of(&fixture(), request());
        let object = |identity: &str| FiniteObject {
            identity: identity.to_owned(),
            type_id: "node".to_owned(),
            snapshot_id: "s".to_owned(),
        };
        let edits: Vec<(&str, InputEdit)> = vec![
            ("model_id", Box::new(|i| i.model_id = "other".to_owned())),
            ("source_id", Box::new(|i| i.source_id = "other".to_owned())),
            ("max_objects", Box::new(|i| i.bounds.max_objects = 3)),
            ("max_references", Box::new(|i| i.bounds.max_references = 2)),
            (
                "max_input_bytes",
                Box::new(|i| i.bounds.max_input_bytes = 3),
            ),
            ("input_bytes", Box::new(|i| i.input_bytes = 2)),
            (
                "object identity",
                Box::new(move |i| {
                    i.objects[1] = object("c");
                    i.references[0].target_id = "c".to_owned();
                }),
            ),
            (
                "object type_id",
                Box::new(|i| i.objects[1].type_id = "other".to_owned()),
            ),
            (
                "object snapshot_id",
                Box::new(|i| i.objects[1].snapshot_id = "t".to_owned()),
            ),
            (
                "reference source_id",
                Box::new(|i| i.references[0].source_id = "b".to_owned()),
            ),
            (
                "reference field_id",
                Box::new(|i| i.references[0].field_id = "other".to_owned()),
            ),
            (
                "reference target_id",
                Box::new(|i| i.references[0].target_id = "a".to_owned()),
            ),
            (
                "extra reference",
                Box::new(|i| {
                    i.bounds.max_references = 2;
                    i.references.push(FiniteReference {
                        source_id: "b".to_owned(),
                        field_id: "next".to_owned(),
                        target_id: "a".to_owned(),
                    });
                }),
            ),
        ];
        let mut seen = vec![base];
        for (field, edit) in edits {
            let identity = identity_of(&fixture_with(|_| {}, edit), request());
            assert!(
                !seen.contains(&identity),
                "varying input `{field}` alone must change the case identity"
            );
            seen.push(identity);
        }
        let revised = identity_of(
            &fixture_with(|selection| selection.revision = "r2".to_owned(), |_| {}),
            request(),
        );
        assert!(
            !seen.contains(&revised),
            "the profile selection is part of the identity"
        );
    }

    /// The same graph offered with its objects and references in another order is the same case
    /// (so a second emission is refused), and a genuinely different graph is a different one.
    ///
    /// Trace: FR-015-AC-94, TC-023.
    #[test]
    fn tc_023_identity_is_canonical_over_the_input_population_order() {
        let node = |identity: &str| FiniteObject {
            identity: identity.to_owned(),
            type_id: "node".to_owned(),
            snapshot_id: "s".to_owned(),
        };
        let edge = |source: &str, target: &str| FiniteReference {
            source_id: source.to_owned(),
            field_id: "next".to_owned(),
            target_id: target.to_owned(),
        };
        let population = |objects: Vec<FiniteObject>, references: Vec<FiniteReference>| {
            fixture_with(
                |_| {},
                move |input| {
                    input.bounds.max_objects = 3;
                    input.bounds.max_references = 2;
                    input.objects = objects;
                    input.references = references;
                },
            )
        };
        let request = || reach("s", "a", "b", "next", 2);
        let forward = population(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("b", "c")],
        );
        let permuted = population(
            vec![node("c"), node("a"), node("b")],
            vec![edge("b", "c"), edge("a", "b")],
        );
        let different = population(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("c", "b")],
        );
        assert_eq!(
            identity_of(&forward, request()),
            identity_of(&permuted, request())
        );
        assert_ne!(
            identity_of(&forward, request()),
            identity_of(&different, request())
        );
        let mut emitted = EmittedCorpusIdentities::new();
        emit(&forward, request(), &[], &mut emitted).unwrap();
        let refusal = emit(&permuted, request(), &[], &mut emitted)
            .map_err(refusal_outcome)
            .unwrap_err();
        assert_eq!(refusal.code, std001_code!("kani_corpus_identity_collision"));
    }

    /// Extracts the `#[kani::proof]` function's name from generated harness source, verbatim.
    fn proof_symbol(contents: &str) -> &str {
        let after_fn = contents
            .split("\nfn ")
            .nth(1)
            .expect("generated harness must declare a proof function");
        after_fn
            .split('(')
            .next()
            .expect("proof function name must be followed by its parameter list")
    }
}
