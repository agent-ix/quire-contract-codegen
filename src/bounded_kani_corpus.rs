//! Integrated bounded-Kani corpus generation over Contract IR's validated finite ABI.
//!
//! The Contract IR lowerers remain the semantic authority.  This module owns the codegen-side
//! vertical slice: once one lowering is admitted, it renders the four corpus roles (oracle,
//! strategy, Kani harness, and proof-dependency graph) from the same profile selection, finite
//! input, and declared proof-dependency census.  A non-success outcome returns before any role is
//! emitted.

use std::{collections::BTreeSet, fmt::Write as _};

use qsl_replay::ByteDigest;
use quire_contract_ir::kani::{
    CheckedArithmeticRequest, CollectionQuery, DispatchIndex, FiniteInput, GraphRequest,
    KaniOutcome, KaniOutcomeKind, KaniProfile, ProfileSelection, QueryKind, ValidatedFiniteInput,
};
use serde::{Deserialize, Serialize};

use crate::{
    kani::{dependency_readiness, deterministic_json, normalize_dependencies},
    prepare_bounded_collection_query, prepare_checked_arithmetic, prepare_finite_graph_reaches,
    Artifact, ProofDependencyEdge, ProofDependencyKind, ProofDependencyRequest, ProofReadiness,
};

/// Stable schema identity for [`CorpusProofDependencyGraph`].
///
/// Deliberately distinct from `src/kani.rs`'s `quire.kani-proof-graph/v2`
/// ([`crate::kani::ProofDependencyGraph`], validated against
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
/// `src/kani.rs`'s `quire.kani-proof-graph/v2`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CorpusProofDependencyGraph {
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

/// Registry of the corpus case identities emitted into one output tree.
///
/// A case's identity is the SHA-256 of its canonical request content ([`CaseIdentity`]): the
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
    construct: &'static str,
    profile: &'a ProfileSelection,
    input: &'a FiniteInput,
    request: RequestIdentity<'a>,
    dependencies: &'a [ProofDependencyEdge],
}

/// Every field of a [`BoundedCorpusRequest`], including those the rendered oracle does not read.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum RequestIdentity<'a> {
    Arithmetic {
        source_id: &'a str,
        operator: &'static str,
        left: i128,
        right: i128,
        minimum: i128,
        maximum: i128,
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
        values: &'a [i128],
        max_items: usize,
        kind: QueryKindIdentity,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum QueryKindIdentity {
    ForAllNonNegative,
    ExistsEqual(i128),
}

impl<'a> From<&'a BoundedCorpusRequest> for RequestIdentity<'a> {
    fn from(request: &'a BoundedCorpusRequest) -> Self {
        match request {
            BoundedCorpusRequest::Arithmetic(request) => Self::Arithmetic {
                source_id: request.source_id,
                operator: checked_method(request.operator),
                left: request.left,
                right: request.right,
                minimum: request.minimum,
                maximum: request.maximum,
            },
            BoundedCorpusRequest::Graph(request) => Self::Graph {
                source_id: &request.source_id,
                start_id: &request.start_id,
                target_id: &request.target_id,
                field_id: &request.field_id,
                max_expansions: request.max_expansions,
            },
            BoundedCorpusRequest::Collection(request) => Self::Collection {
                source_id: &request.source_id,
                values: &request.values,
                max_items: request.max_items,
                kind: match request.kind {
                    QueryKind::ForAllNonNegative => QueryKindIdentity::ForAllNonNegative,
                    QueryKind::ExistsEqual(expected) => QueryKindIdentity::ExistsEqual(expected),
                },
            },
        }
    }
}

impl CaseIdentity<'_> {
    /// Lowercase hex SHA-256 over this content's deterministic JSON bytes.
    fn digest(&self) -> String {
        let bytes = deterministic_json(self)
            .expect("a corpus case identity is plain finite data with no fallible conversion");
        format!("{:x}", ByteDigest::of(bytes.as_bytes()))
    }
}

/// Complete codegen result for one supported corpus case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedCorpusCase {
    /// Semantic family selected before lowering.
    pub family: BoundedCorpusFamily,
    /// Exact Contract IR outcome shared by native/oracle/strategy/Kani case execution.
    pub outcome: KaniOutcome,
    /// Generated roles derived from the same profile and finite input selection.
    pub artifacts: BoundedCorpusArtifacts,
}

/// Generates the complete codegen corpus case from one already validated Contract IR input.
///
/// Contract IR performs profile/dispatch/finite-population validation and semantic lowering first.
/// Consequently a refused, invalid, incomplete, or exhausted case returns its original typed
/// outcome and this function emits no artifact.  Generated Kani source uses a concrete case and
/// intentionally contains no `kani::assume` call.
///
/// `dependencies` is the caller-declared proof-dependency census for this exact case. It is
/// validated with the same rules `generate_kani_bundle` applies to its own census
/// (`crate::kani::validate_dependencies`),
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
/// already holds returns `InvalidInput` `kani_corpus_identity_collision` and emits nothing.
pub fn generate_bounded_kani_corpus_case(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    request: BoundedCorpusRequest,
    dependencies: &[ProofDependencyRequest<'_>],
    emitted: &mut EmittedCorpusIdentities,
) -> Result<BoundedCorpusCase, KaniOutcome> {
    if profile.selection != input.input().profile {
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "kani_profile_input_mismatch",
            request.source_id(),
            profile.selection.revision.clone(),
        ));
    }
    let family = request.family();
    let request_source_id = request.source_id().to_owned();
    let revision = profile.selection.revision.clone();
    // The declared census is validated, and its normalized/sorted form is fixed, before any
    // lowering begins: an invalid census is refused before the generator does any semantic work,
    // and the single normalized form computed here is the one the emitted proof-dependency-graph
    // artifact (`render_artifacts`) carries.
    //
    // Validation is two layers. First, the shared rules (`crate::kani::validate_dependencies`)
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
        || crate::kani::validate_dependencies(dependencies, "").is_err();
    if declared_dependencies_invalid {
        // The failing diagnostic's own `path` (which census index) and `message` (which rule) are
        // not carried into this refusal: `KaniOutcome::non_success`'s `source_id`/`context` fields
        // already carry `request_source_id`/`revision` -- the request's own identity, not the
        // census's -- and folding diagnostic detail into either would conflate two different
        // things this outcome identifies. A caller that needs the failing census index re-runs
        // `crate::kani::validate_dependencies` directly over the same census for the full
        // diagnostic list (ir#80 review finding F7).
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "kani_corpus_dependency_invalid",
            request_source_id,
            revision,
        ));
    }
    let normalized_dependencies = normalize_dependencies(dependencies);
    // Computed before lowering consumes the request; recorded only after lowering succeeds.
    let identity = CaseIdentity {
        construct: family.construct(),
        profile: &profile.selection,
        input: input.input(),
        request: RequestIdentity::from(&request),
        dependencies: &normalized_dependencies,
    }
    .digest();
    let (value, oracle_body) = match request {
        BoundedCorpusRequest::Arithmetic(request) => {
            let lowered = prepare_checked_arithmetic(profile, dispatch, input, request)?;
            // Admission establishes the checked arithmetic/definedness property; the numeric
            // result itself is not a Boolean verdict (zero is as valid as any other in-range
            // result).
            (true, render_arithmetic_oracle(&lowered))
        }
        BoundedCorpusRequest::Graph(request) => {
            let lowered = prepare_finite_graph_reaches(profile, dispatch, input, request)?;
            (lowered.reachable, render_graph_oracle(&lowered, input))
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
                quire_contract_ir::kani::QueryKind::ForAllNonNegative => {
                    "values.iter().all(|value| *value >= 0i128)".to_owned()
                }
                quire_contract_ir::kani::QueryKind::ExistsEqual(expected) => {
                    format!("values.iter().any(|value| *value == {expected}i128)")
                }
            };
            (
                lowered.value,
                format!("{{ let values = [{values}]; {predicate} }}"),
            )
        }
    };
    let outcome = if value {
        KaniOutcome::proved(
            request_source_id.clone(),
            profile.selection.revision.clone(),
        )
    } else {
        KaniOutcome::counterexample(
            request_source_id.clone(),
            profile.selection.revision.clone(),
        )
    };
    if !emitted.claim(&identity) {
        return Err(KaniOutcome::non_success(
            KaniOutcomeKind::InvalidInput,
            "kani_corpus_identity_collision",
            request_source_id,
            revision,
        ));
    }
    let name = format!("{}_{identity}", family.label());
    let artifacts = render_artifacts(family, &name, value, &oracle_body, &normalized_dependencies);
    Ok(BoundedCorpusCase {
        family,
        outcome,
        artifacts,
    })
}

const fn checked_method(operator: quire_contract_ir::NumericOperator) -> &'static str {
    match operator {
        quire_contract_ir::NumericOperator::Add => "checked_add",
        quire_contract_ir::NumericOperator::Subtract => "checked_sub",
        quire_contract_ir::NumericOperator::Multiply => "checked_mul",
        quire_contract_ir::NumericOperator::Divide => "checked_div",
        quire_contract_ir::NumericOperator::Remainder => "checked_rem",
    }
}

fn render_arithmetic_oracle(lowered: &quire_contract_ir::kani::ArithmeticLowering) -> String {
    let operator = checked_method(lowered.request.operator);
    format!(
        "{}i128.{operator}({}i128).is_some_and(|value| value >= {}i128 && value <= {}i128)",
        lowered.request.left,
        lowered.request.right,
        lowered.request.minimum,
        lowered.request.maximum,
    )
}

fn render_graph_oracle(
    lowered: &quire_contract_ir::kani::GraphLowering,
    input: &ValidatedFiniteInput,
) -> String {
    let mut edges = input
        .input()
        .references
        .iter()
        .filter(|edge| edge.field_id == lowered.request.field_id)
        .map(|edge| format!("({:?}, {:?})", edge.source_id, edge.target_id))
        .collect::<Vec<_>>();
    edges.sort();
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

/// Renders one corpus case's artifacts, all named from the case's `name`. Infallible: the caller
/// ([`generate_bounded_kani_corpus_case`]) has already run every fallible step.
///
/// `dependencies` is the already-validated, already-normalized declared census (see
/// [`normalize_dependencies`]).
fn render_artifacts(
    family: BoundedCorpusFamily,
    name: &str,
    value: bool,
    oracle_body: &str,
    dependencies: &[ProofDependencyEdge],
) -> BoundedCorpusArtifacts {
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
    let _ = writeln!(harness, "}}");
    let proof_graph_value = CorpusProofDependencyGraph {
        schema_version: CORPUS_PROOF_GRAPH_SCHEMA.to_owned(),
        proof_id,
        family: label.to_owned(),
        construct: family.construct().to_owned(),
        identity: name.to_owned(),
        readiness: dependency_readiness(dependencies),
        dependencies: dependencies.to_vec(),
    };
    let proof_graph_contents = deterministic_json(&proof_graph_value)
        .expect("a corpus proof-dependency graph is plain finite data with no fallible conversion");
    BoundedCorpusArtifacts {
        oracle: Artifact::new(format!("corpus/{name}.oracle.rs"), oracle),
        strategy: Artifact::new(format!("corpus/{name}.strategy.rs"), strategy),
        kani_harness: Artifact::new(format!("corpus/{name}.kani.rs"), harness),
        proof_graph: Artifact::new(
            format!("corpus/{name}.proof-graph.json"),
            proof_graph_contents,
        ),
    }
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
        FiniteReference, GraphRequest, KaniOutcomeKind, KaniProfile, ModuleDescriptor,
        PopulationCompleteness, ProfileSelection, QueryKind, ResourceBounds, SemanticFamily,
    };
    use quire_contract_ir::NumericOperator;

    use super::{
        generate_bounded_kani_corpus_case, BoundedCorpusRequest, CorpusProofDependencyGraph,
        EmittedCorpusIdentities, ProofReadiness, CORPUS_PROOF_GRAPH_SCHEMA,
    };
    use crate::{ProofDependencyKind, ProofDependencyRequest, ProofDependencyState};

    fn fixture() -> (
        KaniProfile,
        DispatchIndex,
        quire_contract_ir::kani::ValidatedFiniteInput,
    ) {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            abi_revision: "abi".to_owned(),
        };
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
        let input = FiniteInput {
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
        }
        .validate()
        .unwrap();
        (profile, dispatch, input)
    }

    fn arithmetic(source_id: &'static str, left: i128, right: i128) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
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
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(7),
            }),
        ];
        for request in cases {
            // A fresh counter per call: an identical request yields identical output.
            let first = generate_bounded_kani_corpus_case(
                &profile,
                &dispatch,
                &input,
                request.clone(),
                &[],
                &mut EmittedCorpusIdentities::new(),
            )
            .unwrap();
            let second = generate_bounded_kani_corpus_case(
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
            &profile,
            &dispatch,
            &input,
            arithmetic("duplicate-census", 1, 1),
            &[duplicate, duplicate],
            &mut emitted,
        )
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(error.code, "kani_corpus_dependency_invalid");
        let retry = generate_bounded_kani_corpus_case(
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
            &profile,
            &dispatch,
            &input,
            arithmetic("assumed-census", 1, 1),
            &[assumed],
            &mut emitted,
        )
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::InvalidInput);
        assert_eq!(error.code, "kani_corpus_dependency_invalid");
        let retry = generate_bounded_kani_corpus_case(
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

    /// Trace: TC-023.
    #[test]
    fn tc_023_non_success_emits_no_partial_artifacts_or_boolean_claim() {
        let (profile, dispatch, input) = fixture();
        let error = generate_bounded_kani_corpus_case(
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![1, 2],
                max_items: 1,
                kind: QueryKind::ForAllNonNegative,
            }),
            &[],
            &mut EmittedCorpusIdentities::new(),
        )
        .unwrap_err();
        assert_eq!(error.kind, KaniOutcomeKind::ResourceExhausted);
        assert_eq!(error.boolean_claim(), None);
    }

    /// Trace: TC-023.
    #[test]
    fn tc_023_unreachable_graph_request_classifies_as_false() {
        let (profile, dispatch, input) = fixture();
        let generated = generate_bounded_kani_corpus_case(
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
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
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
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Arithmetic(quire_contract_ir::kani::CheckedArithmeticRequest {
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
            &profile,
            &dispatch,
            &input,
            BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
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
        fixture: &(
            KaniProfile,
            DispatchIndex,
            quire_contract_ir::kani::ValidatedFiniteInput,
        ),
        request: BoundedCorpusRequest,
        dependencies: &[ProofDependencyRequest<'_>],
        emitted: &mut EmittedCorpusIdentities,
    ) -> Result<super::BoundedCorpusCase, quire_contract_ir::kani::KaniOutcome> {
        let (profile, dispatch, input) = fixture;
        generate_bounded_kani_corpus_case(profile, dispatch, input, request, dependencies, emitted)
    }

    fn collection(max_items: usize, source_id: &str) -> BoundedCorpusRequest {
        BoundedCorpusRequest::Collection(quire_contract_ir::kani::CollectionQuery {
            source_id: source_id.to_owned(),
            values: vec![2, 2, 7],
            max_items,
            kind: QueryKind::ExistsEqual(7),
        })
    }

    /// A case's name, paths and `#[kani::proof]` symbol come from its request content, so the
    /// same request yields the same ones in any emission order and in any run, and the symbol
    /// carries the name its artifact paths carry.
    ///
    /// Trace: TC-023.
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
    /// Trace: TC-023.
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
            let refusal = emit(&fixture, arithmetic("same", 1, 1), &[], &mut emitted).unwrap_err();
            assert_eq!(refusal.kind, KaniOutcomeKind::InvalidInput);
            assert_eq!(refusal.code, "kani_corpus_identity_collision");
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
